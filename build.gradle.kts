import com.github.jengelman.gradle.plugins.shadow.tasks.ShadowJar

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
    implementation("io.github.cdimascio:dotenv-java:3.0.0")
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
    sourceCompatibility = "1.8"
}

tasks {
    build {
        dependsOn(shadowJar)
    }
}