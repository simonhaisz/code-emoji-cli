if (process.argv.length <= 2) {
	throw new Error('Must provide number for fibonacci sequence')
}
const n = process.argv[2]
fibonacci(n)

function fibonacci(n) {
	if (n <= 0) {
		throw new Error('Must provide a positive number for fibonacci sequence')
	} else if (n === 1) {
		return 0
	} else if (n === 2) {
		return 1
	} else {
		return fibonacci(n - 1) + fibonacci(n - 2)
	}
}