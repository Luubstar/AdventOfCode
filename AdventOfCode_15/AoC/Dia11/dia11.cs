using commons;
class Dia11{
    public static string Execute(){
        string input = Commons.ReadInput("Dia11/dia11.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part1(Part1(input));
    }

    private static string Part1(string input){
        string v = input;
        bool end;
        do{
            v = Wrap(v);
            char prev = ' ';
            char last = ' ';

            bool escalera = false;
            List<string> duos = new List<string>();


            if(!v.Contains("o") && !v.Contains("i") && !v.Contains("l")){
                foreach(char current in v){
                    if(current == last+1 && last == prev+1){escalera = true;}
                    else if(last == current && last != prev){duos.Add(last + current + "");}
                    else if(last == current && last == prev){duos.Remove(prev + last + "");}
                    prev = last;
                    last = current;
                }
            }
            end = escalera&&duos.Count >= 2;
        }
        while(!end);
        return v;
    }

    public static string Wrap(string v){
        if(v[v.Length - 1] == 'z'){return Wrap(v.Substring(0, v.Length - 1)) + "a";}
        else{return v.Substring(0, v.Length - 1) + ((char) (v[v.Length -1] + 1));}
    }

}