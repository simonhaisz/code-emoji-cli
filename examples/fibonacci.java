public class Fibonacci { 
    public static void main (String args[])  { 
		if (args.length < 1) {
			throw new RuntimeException("Must provide number for fibonacci sequence");
		}
		int n = Integer.parseInt(args[0]);
		System.out.println(fibonacci(n));
    }

	static int fibonacci(int n) {
		if (n <= 0) {
			throw new RuntimeException("Must provide a positive number for fibonacci sequence");
		} else if (n === 1) {
			return 0;
		} else if (n === 2) {
			return 1;
		} else {
			return fibonacci(n - 1) + fibonacci(n - 2);
		}
    } 
}