from sympy import solve, symbols

if __name__ == '__main__':
    my_symbols = symbols('x y z r s t u v w')
    x, y, z, r, s, t, u, v, w = my_symbols
    equations = [
        x+r*u-277884961010842-254*r,
        y+r*v-175505292281521-319*r,
        z+r*w-178142491715369+117*r,

        x+s*u-283566174834691-127*s,
        y+s*v-323964410438583+467*s,
        z+s*w-66367418575791-561*s,

        x+t*u-292968982192924-24*t,
        y+t*v-251621777313874-26*t,
        z+t*w-229787798929295-5*t,
    ]

    soln = solve(equations, my_symbols, dict=True)[0]

    print(', '.join(f'{sym}: {soln[sym]}' for sym in [x, y, z]))
    print(f"answer = {soln[x] + soln[y] + soln[z]}")
