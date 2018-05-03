import sys
import collections

def bfs_helper(vertex, search_space, max_depth, current_depth):
    if current_depth<=max_depth:
        if vertex is not None:
            search_space.append(vertex)
        for i in (1, 0):
            bfs_helper(i, search_space, max_depth, current_depth+1)


def bfs(depth):
    search_space = list()
    bfs_helper(None, search_space, depth, 0)
    print(search_space)

def bfs_stack(max_depth):
    closed_stack = list()
    open_stack   = list()

    Vertex = collections.namedtuple('Vertex', ['id', 'depth', 'path'])

    open_stack.append(Vertex(0, 0, [0]))
    open_stack.append(Vertex(1, 0, [1]))

    while True:
        if len(open_stack) == 0:
            break

        last_item = open_stack.pop()

        for i in (0, 1):
            depth = last_item.depth + 1
            if depth <= max_depth:
                lst = list(last_item.path)
                lst.append(i)
                open_stack.append(Vertex(i, depth, lst))

        closed_stack.append(last_item)


#    path_to_end = filter(lambda x:x.depth+1==max_depth, closed_stack)
#    filtered    = map(lambda x: x.path, path_to_end)
#    print(list(filtered))
    print(closed_stack)

if __name__ == '__main__':
    #bfs(int(sys.argv[1]))
    bfs_stack(int(sys.argv[1]))


