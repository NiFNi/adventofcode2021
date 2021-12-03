package Day02.Part1

import getLinesFromInput

fun main() {
    val linesFromInput: List<String> = getLinesFromInput("inputDay2")

    val (x, y) = linesFromInput.map {
        parse(it)
    }.fold(Position(0, 0)) { acc, command ->
        command.apply(acc)
    }

    println(x)
    println(y)
    println(x * y)
}

fun parse(value: String): Command {
    val split = value.split(" ")
    return when (split[0]) {
        "forward" -> Forward(split[1].toInt())
        "up" -> Up(split[1].toInt())
        "down" -> Down(split[1].toInt())
        else -> throw IllegalStateException()
    }
}

data class Position(val x: Int, val y: Int)


sealed class Command(val value: Int) {
    abstract fun apply(position: Position): Position
}

class Forward(value: Int) : Command(value) {
    override fun apply(position: Position): Position = Position(
        position.x + value, position.y
    )
}

class Up(value: Int) : Command(value) {
    override fun apply(position: Position): Position = Position(
        position.x, position.y - value
    )
}

class Down(value: Int) : Command(value) {
    override fun apply(position: Position): Position = Position(
        position.x, position.y + value
    )
}
