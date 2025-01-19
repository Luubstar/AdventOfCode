using commons;
class Dia20{    
    public static string Execute(){
        string input = Commons.ReadInput("Dia20/dia20.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part2(input);
    }

    private static string Part1(string input){
        int v = int.Parse(input);
        int y = 0;
        int calculado = 0;

        while(calculado < v && y < 1000000){
            y++;
            calculado = Divisores(y).Sum() * 10;
        }

        return y.ToString();
    }

    private static string Part2(string input){
        int v = int.Parse(input);
        int y = 0;
        int calculado = 0;

        while(calculado < v && y < 1000000){
            y++;
            calculado = Divisores(y).Where(t => y/t <= 50).Sum() * 11;
        }

        return y.ToString();
    }

    public static List<int> Divisores(int y){
        List<int> divisores = new List<int>();
        List<int> bigDivisores = new List<int>();
        for(int i = 1; i <= Math.Sqrt(y) + 1;i++){
            if(y % i == 0){divisores.Add(i);}
        }

        foreach(int d in divisores){
            if(y != d*d){bigDivisores.Add(y/d);} 
        }

        List<int> res = new List<int>();
        foreach(int v in divisores){if(!res.Contains(v)){res.Add(v);}}
        foreach(int v in bigDivisores){if(!res.Contains(v)){res.Add(v);}}

        return res;
    }   
}