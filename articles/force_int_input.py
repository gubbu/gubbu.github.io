
parsed_integer: int
while True:
    userinput: str = input("please enter a number!")
    try:
        parsed_integer = int(userinput)
        break
    except:
        print(f"an Error was thrown, while parsing your input. please try again.")

print(f"{parsed_integer = }")