import java.io.File
import java.io.InputStream

fun main() {
    val inputStream: InputStream = File("input").inputStream()
    val intList = mutableListOf<Int>()
    inputStream.bufferedReader().forEachLine { intList.add(it.toInt()) }

    val sumList = intList.mapIndexed { index, value ->
        if (index <= 1) {
            value
        } else {
            value + intList[index - 1] + intList[index - 2]
        }
    }.subList(2, intList.size)

    println(sumList.filterIndexed { index, value ->
        index > 0 && value > sumList[index - 1]
    }.count())
}