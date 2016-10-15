#include <stdlib.h>
#include <stdio.h>
#include <string.h>

int L, H;

char **map;
int *tasks;
int tasks_n, tasks_l = 5000000;

int calcsurf() {
    int ret = 0;
    while(tasks_n > 0) {
        int t_i = tasks_n * 2;
        int x = tasks[t_i - 2], y = tasks[t_i - 1];
        tasks_n -= 1;
        t_i -= 2;
        if(x < 0 || x >= L || y < 0 || y >= H || map[y][x] != 'O') {
            continue;
        }
        else {
            ret += 1;
            map[y][x] = 'X';
            tasks[t_i + 0] = x - 1;
            tasks[t_i + 1] = y;
            tasks[t_i + 2] = x;
            tasks[t_i + 3] = y - 1;
            tasks[t_i + 4] = x + 1;
            tasks[t_i + 5] = y;
            tasks[t_i + 6] = x;
            tasks[t_i + 7] = y + 1;
            tasks_n += 4;
            if (tasks_n > tasks_l - 10) {
                tasks_l *= 2;
                tasks = (int*)realloc(tasks, sizeof(int) * tasks_l);
            }
        }
    }
    return ret;
}

void rewind_map() {
    for(int y = 0; y < H; y += 1)
        for(int x = 0; x < L; x += 1)
            if(map[y][x] == 'X')
                map[y][x] = 'O';
}

int main() {
    scanf("%d", &L);
    scanf("%d", &H); fgetc(stdin);
    map = (char**)malloc(sizeof(char*) * H);
    tasks = (int*)malloc(sizeof(int) * tasks_l);
    for (int i = 0; i < H; i++) {
        map[i] = (char*)malloc(sizeof(char*) * L + 2);
        fgets(map[i], L + 2, stdin);
    }
    int N;
    scanf("%d", &N);
    for (int i = 0; i < N; i++) {
        int X;
        int Y;
        scanf("%d%d", &X, &Y);
        tasks[0] = X;
        tasks[1] = Y;
        tasks_n = 1;
        printf("%i\n", calcsurf());
        rewind_map();
    }
    return 0;
}