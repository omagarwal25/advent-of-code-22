fun main() {
    fun part1(input: List<String>): Int {
        val processed = input.map { if (it == "") 0 else it.toInt() }

        val calories = mutableListOf<Int>()
        var current = 0

        for (i in processed) {
            if (i == 0) {
                calories.add(current)
                current = 0
            } else {
                current += i
            }
        }

        return calories.max()
    }

    fun part2(input: List<String>): Int {
        val processed = input.map { if (it == "") 0 else it.toInt() }

        val calories = mutableListOf<Int>()
        var current = 0

        for (i in processed) {
            if (i == 0) {
                calories.add(current)
                current = 0
            } else {
                current += i
            }
        }

        calories.sortDescending()

        return calories.take(3).sum()
    }

    // test if implementation meets criteria from the description, like:
    val testInput = readInput("Day01_test")
    check(part1(testInput) == 67016)

    val input = readInput("Day01")
    println(part1(input))
    println(part2(input))
}
