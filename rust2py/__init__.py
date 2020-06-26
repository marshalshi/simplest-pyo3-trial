from .rust2py import *


def loop_py(n):
    """
    parameter
    ---------
    n: int, number of loops

    O(N^2) complex sample code
    """
    for i in range(n):
        for j in range(n):
            j - i
