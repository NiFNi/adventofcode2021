package Day02.Part2

import getLinesFromInput

fun main() {
    val linesFromInput: List<String> = getLinesFromInput("inputDay2")
    val position = linesFromInput.map {
        parse(it)
    }.fold(Position(0, 0, 0)) { acc, command ->
        command.apply(acc)
    }
    println(position)
    println(position.position * position.depth)
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

data class Position(val position: Int, val depth: Int, val aim: Int)

sealed class Command(val value: Int) {
    abstract fun apply(position: Position): Position
}

class Forward(value: Int) : Command(value) {
    override fun apply(position: Position): Position = Position(
        position.position + value, position.depth + (position.aim * value), position.aim
    )
}

class Up(value: Int) : Command(value) {
    override fun apply(position: Position): Position = Position(
        position.position, position.depth, position.aim - value
    )
}

class Down(value: Int) : Command(value) {
    override fun apply(position: Position): Position = Position(
        position.position, position.depth, position.aim + value
    )
}
