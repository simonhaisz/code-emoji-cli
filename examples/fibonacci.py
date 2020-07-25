import sys

if len(sys.argv) < 2:
	raise Exception("Must provide number for fibonacci sequence")

def Fibonacci(n): 
    if n < 0: 
        print("Must provide a positive number for fibonacci sequence") 
    elif n == 1: 
        return 0
    elif n == 2: 
        return 1
    else: 
        return Fibonacci(n - 1) + Fibonacci(n - 2)
