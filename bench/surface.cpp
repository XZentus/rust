#include <iostream>
#include <cstdlib>
#include <vector>
#include <utility>
#include <limits>

using namespace std;

int L, H;

vector<pair<int, int>> tasks;
vector<vector<int>> map;

int calcsurf(int mark) {
    int ret = 0;
    while(!tasks.empty()) {
        int x = tasks.back().first, y = tasks.back().second;
        tasks.pop_back();
        if(map[y][x] == 0 || map[y][x] == mark) {
            continue;
        }
        else {
            ret += 1;
            map[y][x] = mark;
            if(x > 0)
                tasks.push_back(make_pair(x - 1, y));
            if(y > 0)
                tasks.push_back(make_pair(x, y - 1));
            if(x < L - 1)
                tasks.push_back(make_pair(x + 1, y));
            if(y < H - 1)
                tasks.push_back(make_pair(x, y + 1));
        }
    }
    return ret;
}

int main() {
    cin >> L >> H;
    map.reserve(H);
    string str;
    cin.ignore(numeric_limits<streamsize>::max(), '\n');
    for(int i = 0; i < H; i += 1) {
        map[i].reserve(L);
        getline(cin, str);
        for(int j = 0; j < L; j += 1) {
            if(str[j] == '#')
                map[i].push_back(0);
            else
                map[i].push_back(1);
        }
    }
    int N;
    cin >> N;
    for (int i = 0; i < N; i += 1) {
        int x, y;
        cin >> x >> y;
        tasks.push_back(make_pair(x, y));
        cout << calcsurf(i + 2) << endl;
    }
    return 0;
}
