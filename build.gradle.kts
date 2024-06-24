plugins {
    application
    id("com.github.johnrengelman.shadow") version "7.1.2"
}

application.mainClass = "de.leghast.dalliklick.DalliKlickBot"
group = "de.leghast"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(platform("org.junit:junit-bom:5.10.0"))
    testImplementation("org.junit.jupiter:junit-jupiter")
    implementation("org.slf4j:slf4j-api:2.0.13")
    implementation("ch.qos.logback:logback-classic:1.4.12")
    implementation("com.moandjiezana.toml:toml4j:0.7.2")
    implementation("com.google.code.gson:gson:2.11.0")
    implementation("com.google.guava:guava:32.0.0-android")
    implementation("com.surrealdb:surrealdb-driver:0.1.0")
    implementation("org.java-websocket:Java-WebSocket:1.5.6")
    implementation("com.intuit.fuzzymatcher:fuzzy-matcher:1.2.1")
    implementation("net.dv8tion:JDA:5.0.0-beta.23") { // replace $version with the latest version
        // Optionally disable audio natives to reduce jar size by excluding `opus-java`
        // Gradle DSL:
        // exclude module: 'opus-java'
        // Kotlin DSL:
        // exclude(module="opus-java")
    }
}

tasks.test {
    useJUnitPlatform()
}

tasks.withType<JavaCompile> {
    options.encoding = "UTF-8"
    options.isIncremental = true

    // Set this to the version of java you want to use,
    // the minimum required for JDA is 1.8
    sourceCompatibility = "17"
}

tasks {
    build {
        dependsOn(shadowJar)
    }
}