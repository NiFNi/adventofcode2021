import java.io.File
import java.io.InputStream

fun main() {
    val inputStream: InputStream = File("input").inputStream()
    val intList = mutableListOf<Int>()
    inputStream.bufferedReader().forEachLine { intList.add(it.toInt()) }

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