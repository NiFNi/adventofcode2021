fun main() {
    val linesFromInput = getLinesFromInput("inputDay1")
    val lineList = linesFromInput.map { it.toInt() }

    println(lineList.filterIndexed { index, value ->
        index > 0 && value > lineList[index - 1]
    }.count())
}