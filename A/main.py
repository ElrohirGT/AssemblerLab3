import re;

BINARY_NUMBER = "^[0-1]{1,}$";
BIT_LEN = 8

def OnesComplement(binary: str) -> str:
    expresion = "";
    for digit in binary:
        expresion += "0" if digit == "1" else "1"; # Swaping binary digits values.
    return expresion;

def TwosComplement(binary: str) -> str:
    complement1 = OnesComplement(binary);
    complement2 = bin( int(complement1, 2) + 1)
    return str(complement2)[2:] 

def isBinary(expression: str) -> bool:
    return re.match(BINARY_NUMBER, expression);

def hasExactBits(expression:str, expectedLenght: int) -> bool:
    return len(expression) == expectedLenght

def hasEnoughBits(expression:str, expectedLenght: int) -> bool:
    return len(expression) >= expectedLenght

def complementZeros(binary:str, expectedLenght: int) -> str:
    if not hasEnoughBits(binary, expectedLenght):
        neededZeros = expectedLenght - len(binary)
        return "0"*neededZeros + binary;
    else:
        return binary[-expectedLenght:]

if __name__ == "__main__":
    binaryInput = input("Enter a valid 8 bit binary:\n");
    if not isBinary(binaryInput) :
        print("\nMjm that do NOT SEEMS LIKE A BINARY...")
    elif not hasExactBits(binaryInput, BIT_LEN):
        print("\nMjm, that's do not have 8 BITS")
    else:
        print("C1: " 
              + complementZeros(
                OnesComplement(binaryInput), 
                BIT_LEN))
        print("C2: " 
            + complementZeros(
            TwosComplement(binaryInput), 
            BIT_LEN))