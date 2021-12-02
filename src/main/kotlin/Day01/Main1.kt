import java.io.File
import java.io.InputStream

fun main() {
    val inputStream: InputStream = File("input").inputStream()
    val lineList = mutableListOf<Int>()
    inputStream.bufferedReader().forEachLine { lineList.add(it.toInt()) }

    println(lineList.filterIndexed { index, value ->
        index > 0 && value > lineList[index - 1]
    }.count())
}