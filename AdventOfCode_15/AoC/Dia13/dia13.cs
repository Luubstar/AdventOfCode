using commons;
class Dia13{
    public static string Execute(){
        string[] input = Commons.ReadLines("Dia13/dia13.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part2(input);
    }

    private static string Part1(string[] input){
        Grafo g = new Grafo();
        foreach(string line in input){
            string[] separadores = line.Split(" ");
            string o = separadores[0];
            int v = 0;
            if(separadores[2] == "lose"){v = - int.Parse(separadores[3]);}
            else{v = int.Parse(separadores[3]);}
            string d = separadores[10].Substring(0, separadores[10].Length-1);
            g.AddConnection(o, d, v);
        }
        List<string> l = new List<string>(g.GetNodes());

        return MejorPosicion(Permutar(l), g).ToString();
    }

    private static string Part2(string[] input){
        Grafo g = new Grafo();
        foreach(string line in input){
            string[] separadores = line.Split(" ");
            string o = separadores[0];
            int v = 0;
            if(separadores[2] == "lose"){v = - int.Parse(separadores[3]);}
            else{v = int.Parse(separadores[3]);}
            string d = separadores[10].Substring(0, separadores[10].Length-1);
            g.AddConnection(o, d, v);
        }
        
        foreach(string n in g.GetNodes()){
            g.AddConnection("Yo", n, 0);
            g.AddConnection(n, "Yo", 0);
        }
        
        List<string> l = new List<string>(g.GetNodes());

        return MejorPosicion(Permutar(l), g).ToString();
    }

    private static List<List<T>> Permutar<T>(List<T> lista)
    {
        var resultado = new List<List<T>>();

        if (lista.Count == 1)
        {
            resultado.Add(new List<T>(lista));
            return resultado;
        }

        foreach (var elemento in lista)
        {
            var subLista = new List<T>(lista);
            subLista.Remove(elemento);

            foreach (var permutacion in Permutar(subLista))
            {
                permutacion.Insert(0, elemento);
                resultado.Add(permutacion);
            }
        }

        return resultado;
    }

    public static int MejorPosicion(List<List<string>> listas, Grafo g){
        int max = 0;
        Dictionary<string, Dictionary<string, int>> valores = g.GetGraph();

        foreach(List<string> perms in listas){
            string o = perms[0];
            perms.Remove(o);
            perms.Add(o);
            int value = 0;

            foreach(string d in perms){
                value += valores[o][d];
                value += valores[d][o];
                o = d;
            }
            max = Math.Max(value, max);
        }

        return max;
    }

}