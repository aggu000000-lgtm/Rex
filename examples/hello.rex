make message = "Hello from Rex!"
make number = 42

show message
show number

make x = 10
make y = 20
make sum = x + y
show sum

if sum > 25 {
    show "Sum is greater than 25"
}

func add a b {
    make result = a + b
    return result
}

make result = add 5 10
show result