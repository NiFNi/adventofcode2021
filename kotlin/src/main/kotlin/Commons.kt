fun getResourceAsText(path: String): String {
    return object {}.javaClass.getResource(path).readText()
}

fun getLinesFromInput(inputName: String): List<String> {
    return getResourceAsText(inputName).split('\n').filter { it.isNotBlank() }
}

fun getLinesFromInputWithBlank(): List<String> {
    return getResourceAsText("input").split('\n')
}