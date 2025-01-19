using System.Text;
using commons;
class Dia10{
    public static string Execute(){
        string input = Commons.ReadInput("Dia10/dia10.txt");
        string res = Part1(input);
        return "Primera parte y segunda: " + res;
    }

    private static string Part1(string input){
        StringBuilder cadena = new StringBuilder().Append(input);
        int l = 0;
        for(int i = 0; i < 50; i++){
            char lastNumber = cadena[0];
            int reps = 1;
            StringBuilder res = new StringBuilder();

            for(int j = 1; j < cadena.Length; j++){
                if(cadena[j] != lastNumber){
                    res.Append(reps + "" + lastNumber);
                    lastNumber = cadena[j]; 
                    reps = 1;
                }
                else if (cadena[j] == lastNumber){reps++;}
            }
            cadena = res.Append(reps + "" + lastNumber);
            if (i == 39){l = cadena.Length;}
        }

        return l + "|" + cadena.Length.ToString();
    }

}