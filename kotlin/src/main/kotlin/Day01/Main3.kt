fun main() {
    val linesFromInput = getLinesFromInput("inputDay1")
    val intList = linesFromInput.map { it.toInt() }

    val sumList = mutableListOf<Int>()

    intList.forEachIndexed { index, value ->
        if (index > 1) {
            sumList.add(intList[index - 2] + intList[index - 1] + value)
        }
    }

    println(sumList.filterIndexed { index, value ->
        index > 0 && value > sumList[index - 1]
    }.count())
}