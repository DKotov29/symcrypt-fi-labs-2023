N1 = 222
C1 = 71
N2 = 229
C2 = 73
size = 2048

L1 = []
L2 = []
L3 = []

array = list("10010110111100000100100000001111001100110110000010001111010111000110101010011001011000011001010101010000101100111000011110000110001000111111100101111010000011010011011110001000000110100101011110100000000111001010100101000011110111101110001001011010110011110001110101001111000101001001111010011010110011101010000010101010001011001101011100101010010111000011001010011010101011101011101001111000100001010110110001000011011101111000011101000111000101111101001000000010010001010101111010001010001001000001010110011111100010001001011011000011111000100010111100011000110000011110100111001110110010111111111110011011011100101100001000111100111000110001011001010000111111001100110100000111000011010010010011100111111000111000000010110001000101011001000011011011110111001000101100000100001010000100100110100010101000110010001101011110100110011001100011000011101011111101000010100101000110101011101011001100001110001111011000110000111011011101101101111011110100111101101001000010110001110110111111001110111011001011110010101110110000100001001011001101011111101001101101111101011000010011111000010010011101000100000001101101110110010000111100001011010110110011001000100000110111100001101100000001100100000010000111011110111000101011110011001011101101011001010110101101010101111111011110111101110100011001101101100111010010101011110000001010101001000001000111111011100000011000110110001010110010100101011010000011110101111101101001100100001101011100111000100011110000001010001000101010110110100001001110001101101000000110011000110000100001101010010011010101100011011111100000111110100100001000110000010111101110101101010101000001100000101111001000010111000010111000011111111111101001111101111100110011000011100110000001111010101101100010000111001100001001111010101101010100111110010000001010111000001001001001101111011000100010101011111010101011010111010111111110111011011000100011110001111011110011101011000001100010101110001001110001011101110110000010101010110101101011011001110110011110011000111111000011111011111010111011111000101100100100001110010100101100")

def Geffe(l1, l2, l3):
    sequence = [0] * 28
    for i in range(27):
        sequence[i] = (l3[i] & l1[i]) ^ ((1 ^ l3[i]) & l2[i])
    return sequence

def isSim(first, second):
    return all(first[i] == second[i] for i in range(len(first)))

def plus1(array):
    carry = 1
    for i in range(len(array) - 1, -1, -1):
        array[i] += carry

        if array[i] == 2:
            array[i] = 0
            carry = 1
        else:
            carry = 0
            break
    if carry == 1:
        array.insert(0, carry)

    return array

def L1Calc(n, L1):
    a = n.copy()
    n1 = n.copy()
    for i in range(26, N1):
        a[i] = a[i - 22] ^ a[i - 25]
    r = sum(a[i] ^ array[i] for i in range(N1))
    if r < C1:
        for i in range(25, 28):
            n1[i] = a[i]
        L1.append([n1, r])

def L2Calc(n, L2):
    a = n.copy()
    n2 = n.copy()
    for i in range(26, N2):
        a[i] = a[i - 20] ^ a[i - 24] ^ a[i - 25] ^ a[i]
    r = sum(a[i] ^ array[i] for i in range(N2))
    if r < C2:
        for i in range(26, 28):
            n2[i] = a[i]
        L2.append([n2, r])

a = [0] * 28
for i in range(2 ** 26):
    L2Calc(a, L2)
    if i < 2 ** 25:
        L1Calc(a, L1)
    a = plus1(a)


def L3Calc(n):
    n1 = n.copy()
    for l1_item in L1:
        for l2_item in L2:
            gen = Geffe(l1_item[0], l2_item[0], n)
            if isSim(array, gen):
                L3.append(n1)

a = [0] * 28
for i in range(2 ** 27):
    L3Calc(a)
    a = plus1(a)