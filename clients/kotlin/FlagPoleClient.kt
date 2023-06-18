import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue
import mu.KotlinLogging
import org.http4k.core.HttpHandler
import org.http4k.core.Method
import org.http4k.core.Request
import org.http4k.core.Response
import org.http4k.core.Status
import java.time.Duration
import kotlin.time.toKotlinDuration
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Semaphore

// This code example uses dependencies
// implementation(platform("org.http4k:http4k-bom:4.48.0.0"))
// implementation("org.http4k:http4k-client-okhttp")
// implementation("com.fasterxml.jackson.core:jackson-core:2.15.2")
// implementation("io.github.microutils:kotlin-logging-jvm:3.0.5")
// implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.7.1")

interface FeatureFlagClient {
    suspend fun getFlags(namespace: String): FlagState
    suspend fun eTag(namespace: String): String
}

data class FlagState(
    val flags: Set<String>,
    val eTag: String
)

class FeatureFlagClientImpl(
    private val host: String,
    private val client: HttpHandler,
    private val mapper: ObjectMapper,
): FeatureFlagClient {
    override suspend fun getFlags(namespace: String): FlagState {
        val request = Request(Method.GET, "$host/api/flags/$namespace").header("accept", "application/json")
        val response: Response = client(request)
        check(response.status == Status.OK) { "Received HTTP status ${response.status}" }
        data class ResponseBody(val flags: Set<String>)
        check(response.body.length != 0L) { "Received empty body in response" }
        val responseBody: ResponseBody = mapper.readValue(response.bodyString())
        val eTag: String = response.header("etag")!!
        return FlagState(
            flags = responseBody.flags,
            eTag = eTag
        )
    }

    override suspend fun eTag(namespace: String): String {
        val request = Request(Method.HEAD, "$host/api/flags/$namespace")
        val response: Response = client(request)
        check(response.status == Status.OK) { "Received HTTP status ${response.status}" }
        return response.header("etag")!!
    }
}

class FeatureFlagService(
    private val namespace: String,
    private val refreshInterval: Duration,
    private val client: FeatureFlagClient,
) {
    private val log = KotlinLogging.logger {}
    private val semaphore = Semaphore(1)
    private var flags: Set<String> = emptySet()
    private var eTag: String? = null

    /**
     * Call this method to check if a flag is enabled or not
     */
    suspend fun isEnabled(flag: String): Boolean {
        startOnce()
        return flag in this.flags
    }

    private suspend fun startOnce() = coroutineScope {
        if (semaphore.tryAcquire()) {
            launch { loop() }
        }
    }

    private tailrec suspend fun loop() {
        try {
            val eTag: String = client.eTag(namespace)
            if (eTag != this.eTag) {
                val state: FlagState = client.getFlags(namespace)
                this.flags = state.flags
                this.eTag = state.eTag
            }
        } catch (e: Exception) {
            log.warn(e) { "Request to flagpole server failed" }
        }
        delay(refreshInterval.toKotlinDuration())
        loop()
    }
}

