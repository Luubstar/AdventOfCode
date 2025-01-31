using commons;
class Dia19{    
    public static string Execute(){
        string[] input = Commons.ReadLines("Dia19/dia19.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part2(input);
    }

    private static string Part1(string[] input){
        List<(string, string)> moleculas = new List<(string,string)>();
        HashSet<string> unicas = new HashSet<string>();
        int p = 0;

        foreach(string linea in input){
            if(linea == ""){p++; break;}
            string[] segmentos = linea.Split(" => ");
            moleculas.Add((segmentos[0], segmentos[1]));
            p++;
        }
        List<string> bases = Separar(input[p], moleculas);

        int i = 0;
        foreach(string c in bases){
            foreach((string name, string rep) m in moleculas){
                if(m.name == c){
                    List<string> r = new List<string>(bases);
                    r[i] = m.rep;
                    unicas.Add(string.Join("", r));
                }
            }
            i++;
        }

        return unicas.Count.ToString();
    }

    private static string Part2(string[] input){
        int p = 0;
        foreach(string linea in input){
            if(linea == ""){p++; break;}
            p++;
        }

        string molecula = input[p];

        return (molecula.Count(char.IsUpper) - CountOccurrences(molecula, "Rn") -CountOccurrences(molecula, "Ar") - 2 * CountOccurrences(molecula, "Y")  -1).ToString();
        
        
    }

    static int CountOccurrences(string text, string substring)
    {
        int count = 0;
        int index = 0;
        
        while ((index = text.IndexOf(substring, index)) != -1)
        {
            count++;
            index += substring.Length;
        }
        
        return count;
    }

    private static List<string> Separar(string entrada, List<(string,string)> moleculas){
        List<string> res = new List<string>();
        int i = 0;
        string back = "";
        while (i <entrada.Length)
        {
            bool found = false;
            foreach ((string m, string) molecule in moleculas)
            {
                if (entrada.Substring(i).StartsWith(molecule.m))
                {
                    if(back != "") {res.Add(back);}
                    res.Add(molecule.m);
                    i += molecule.m.Length;
                    back = "";
                    found = true;
                    break;
                }
            }

            if (!found)
            {
                back += entrada[i];
                i++;
            }
        }

        return res;
    }
}