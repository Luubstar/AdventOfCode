using commons;
class Dia1
{
    public static string Execute(){
        string input = Commons.ReadInput("Dia1/dia1.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part2(input);
    }

    private static string Part1(string input){
        int res = 0;
        
        foreach(char c in input){
            if (c == ')'){res--;}
            else if (c == '('){res++;}
        }

        return res.ToString();
    }

    private static string Part2(string input){
        int res = 0;
        int pos = 1;

        foreach(char c in input){
            if (c == ')'){res--;}
            else if (c == '('){res++;}

            if(res == -1){break;}
            pos++;
        }

        return pos.ToString();
    }

}