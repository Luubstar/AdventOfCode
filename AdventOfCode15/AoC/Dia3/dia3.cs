
using commons;
class Dia3{
    public static string Execute(){
        string input = Commons.ReadInput("Dia3/dia3.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part2(input);
    }

    private static string Part1(string input){
        List<(int,int)> posiciones = new List<(int, int)>();
        (int x,int y)  pos = (x: 0, y: 0);
        posiciones.Add(pos);

        foreach(char c in input){
            if      (c=='^'){CheckAndAdd((pos.x, ++pos.y), posiciones);}
            else if (c=='v'){CheckAndAdd((pos.x, --pos.y), posiciones);}
            else if (c=='>'){CheckAndAdd((++pos.x, pos.y), posiciones);}
            else if (c=='<'){CheckAndAdd((--pos.x, pos.y), posiciones);}
        }

        return posiciones.Count().ToString();
    }

    private static string Part2(string input){
        List<(int,int)> posiciones = new List<(int, int)>();
        (int x,int y) robotSanta = (x: 0, y: 0);
        (int x,int y) santa = (x: 0, y: 0);
        (int x,int y) pos;
        bool turnoDeSanta = true;
        posiciones.Add(robotSanta);

        foreach(char c in input){
            if      (turnoDeSanta){pos = santa;}
            else    {pos = robotSanta;}

            if      (c=='^'){CheckAndAdd((pos.x, ++pos.y), posiciones);}
            else if (c=='v'){CheckAndAdd((pos.x, --pos.y), posiciones);}
            else if (c=='>'){CheckAndAdd((++pos.x, pos.y), posiciones);}
            else if (c=='<'){CheckAndAdd((--pos.x, pos.y), posiciones);}

            if      (turnoDeSanta){santa = pos;}
            else    {robotSanta = pos;}
            turnoDeSanta = !turnoDeSanta;
        }
        return posiciones.Count().ToString();
    }

    private static void CheckAndAdd((int,int) pos, List<(int,int)> posiciones){
        if(!posiciones.Contains(pos)){posiciones.Add(pos);}
    }


}