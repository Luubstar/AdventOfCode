using commons;
class Dia6{
    public static string Execute(){
        string[] input = Commons.ReadLines("Dia6/dia6.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part2(input);
    }

    private static string Part1(string[] input){
        bool[,] array = new bool[1000,1000];
        foreach(string orden in input){
            (int ox, int oy, int dx, int dy, int acc) action = Parse(orden);
            for(int x = action.ox; x <= action.dx; x++){
                for(int y = action.oy; y <= action.dy; y++){
                    if(action.acc == 0){array[x,y] = false;}
                    else if (action.acc == 1){array[x,y] = true;}
                    else {array[x,y] = !array[x,y];}
                }
            }
        }
        int i = 0;
        foreach(bool value in array){if(value){i++;}}
        return i.ToString();
    }

    private static string Part2(string[] input){
      int[,] array = new int[1000,1000];
        foreach(string orden in input){
            (int ox, int oy, int dx, int dy, int acc) action = Parse(orden);
            for(int x = action.ox; x <= action.dx; x++){
                for(int y = action.oy; y <= action.dy; y++){
                    if(action.acc == 1){array[x,y] += 1;}
                    else if (action.acc == 0){
                        array[x,y] -= 1;
                        if(array[x,y] < 0){array[x,y] = 0;}
                    }
                    else {array[x,y] += 2;}
                }
            }
        }
        int i = 0;
        foreach(int value in array){i+=value;}
        return i.ToString();
    }

    private static (int ox, int oy, int dx, int dy, int acc) Parse(string line){
        int accion;
        string rest;
        if(line.StartsWith("turn off")){accion = 0; rest = line.Split("turn off")[1];}
        else if (line.StartsWith("turn on")){accion = 1; rest = line.Split("turn on")[1];}
        else{accion = 2; rest = line.Split("toggle")[1];}

        string[] values = rest.Split(" ");
        int ox = int.Parse(values[1].Split(",")[0]);
        int oy = int.Parse(values[1].Split(",")[1]);
        int dx = int.Parse(values[3].Split(",")[0]);
        int dy = int.Parse(values[3].Split(",")[1]);

        return (ox,oy,dx,dy,accion);
    }

}