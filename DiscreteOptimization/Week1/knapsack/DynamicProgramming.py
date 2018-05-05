import sys

def dynamic_programming(k, n, weight, value, memoized_item):

    if (n <= 0):
        return 0

    w = weight[n]
    v = value[n]

    try:
        previous_value = memoized_item[(k, n)]
    except KeyError:
        previous_value = dynamic_programming(k, n-1, weight, value, memoized_item)

    if (w  <= k):
        return_value = max(previous_value, v  + dynamic_programming(k-w, n-1, weight, value, memoized_item))
        memoized_item[(k,n)] = return_value
        return return_value
    return previous_value


def main():
    f = open(sys.argv[1], 'r')
    N, K = map(lambda x: int(x), f.readline().lstrip().rstrip().split(" "))
    value, weight = list(), list()
    weight.append(0)
    value.append(0)

    for line in f.readlines():
        v, w = map(lambda x: int(x), line.lstrip().rstrip().split(" "))

        weight.append(w)
        value.append(v)

    dynamic_table = [[0]*(N+1)]*(K+1)

    memoized_item = dict()

#    for n in range(0, N+1):
#        for k in range(0, K+1):
#
#            dynamic_table[k][n] = dynamic_programming(k, n, weight, value, memoized_item)
#

    #for l in dynamic_table:
        #print(l)
    print(dynamic_programming(K, N, weight, value, memoized_item))

if __name__ == '__main__':
    main()



