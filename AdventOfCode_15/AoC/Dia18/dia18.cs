using commons;
class Dia18{
    static int TAMAÑO = 100;
    static int STEPS = 100;
    public static string Execute(){
        string[] input = Commons.ReadLines("Dia18/dia18.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part2(input);
    }

    private static string Part1(string[] input){
        bool[,] mapa = CrearMapa(input, TAMAÑO);

        for(int reps = 0; reps < STEPS; reps++){mapa = DesplazarMapa(mapa);}
        int i = 0;
        foreach(bool b in mapa){if(b){i++;}}
        return i.ToString();
    }

    private static string Part2(string[] input){
        bool[,] mapa = CrearMapa(input, TAMAÑO);

        for(int reps = 0; reps < STEPS; reps++){
            mapa[0,TAMAÑO-1] = true;
            mapa[TAMAÑO-1,0] = true;
            mapa[0,0] = true;
            mapa[TAMAÑO-1,TAMAÑO-1] = true;
            mapa = DesplazarMapa(mapa);   
        }
        
        mapa[0,TAMAÑO-1] = true;
        mapa[TAMAÑO-1,0] = true;
        mapa[0,0] = true;
        mapa[TAMAÑO-1,TAMAÑO-1] = true;

        int i = 0;
        foreach(bool b in mapa){if(b){i++;}}
        return i.ToString();
    }

    public static bool[,] CrearMapa(string[] input, int t){
        bool[,] mapa = new bool[t,t];
        for(int y = 0; y < t; y++){
            string r = input[y];
            for(int x = 0; x < t; x++){
                mapa[x,y] = r[x] == '#';
            }
        }
        return mapa;
    }
    public static bool[,] DesplazarMapa(bool[,] mapa){
        bool[,] now = new bool[TAMAÑO, TAMAÑO];
        for(int y = 0; y < TAMAÑO; y++){
            for(int x = 0; x < TAMAÑO; x++){
                int vecinosActivos = 0;
                if(x-1 >= 0){
                    if(y-1 >= 0 && mapa[x-1,y-1]){vecinosActivos++;}
                    if(mapa[x-1,y]){vecinosActivos++;}
                    if(y+1 < TAMAÑO && mapa[x-1,y+1]){vecinosActivos++; }
                }

                if(y -1 >= 0 && mapa[x, y-1]){vecinosActivos++;}
                if(y +1 < TAMAÑO && mapa[x, y+1]){vecinosActivos++;}
                
                if(x+1 < TAMAÑO){
                    if(y-1 >= 0 && mapa[x+1,y-1]){vecinosActivos++;}
                    if(mapa[x+1,y]){vecinosActivos++;}
                    if(y+1 < TAMAÑO && mapa[x+1,y+1]){vecinosActivos++;}
                }

                if(mapa[x,y] && (vecinosActivos == 2 || vecinosActivos == 3)){now[x,y] = true;}
                else if(!mapa[x,y] && vecinosActivos == 3){now[x,y] = true;}
            }
        }
        return now;
    }
}