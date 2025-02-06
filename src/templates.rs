pub const INJECTION: &str = "
    package <!pkg_name!>.<!module_name!>
    
    import org.koin.core.module.dsl.singleOf
    import org.koin.dsl.module

    object <!module_name_cap!>Injection {
        val inject =
            module {
                singleOf(::<!module_name_cap!>Service)
            }
    }
";

pub const SERVICE: &str = "
    package <!pkg_name!>.<!module_name!>

    import org.koin.core.component.KoinComponent

    class <!module_name_cap!>Service : KoinComponent {}
";

pub const ROUTING: &str = "
    package <!pkg_name!>.<!module_name!>

    import io.ktor.server.application.*
    import io.ktor.server.routing.*
    import org.koin.ktor.ext.inject

    fun Application.configure<!module_name_cap!>Routing() {
        val service by inject<<!module_name_cap!>Service>()

        routing {
            route(\"/<!module_name!>\") {

            }
        }
    }
";
