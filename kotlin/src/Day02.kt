enum class Move(val score: Int) {
    Rock(1), Paper(2), Scissors(3)
}

fun main() {
    fun processUser(c: Char): Move {
        return when (c) {
            'X' -> Move.Rock
            'Y' -> Move.Paper
            'Z' -> Move.Scissors
            else -> Move.Scissors
        }
    }

    fun processComputer(c: Char): Move {
        return when (c) {
            'A' -> Move.Rock
            'B' -> Move.Paper
            'C' -> Move.Scissors
            else -> Move.Scissors
        }
    }

    fun score(user: Move, computer: Move): Int {
        return when {
            user == computer -> 6
            user.score == 1 && computer.score == 3 -> 3
            user.score == 2 && computer.score == 1 -> 3
            user.score == 3 && computer.score == 2 -> 3
            else -> 0
        }
    }


    fun part1(input: List<String>): Int {
        return input.map {
            it.split(" ").map { c -> c[0] }
        }.map { processComputer(it[0]) to processComputer(it[1]) }
            .sumOf { score(it.first, it.second) + it.second.score }

    }

    fun part2(input: List<String>): Int {

    }

    // test if implementation meets criteria from the description, like:
    val testInput = readInput("Day01_test")
    check(part1(testInput) == 67016)

    val input = readInput("Day01")
    println(part1(input))
    println(part2(input))
}
