using commons;
class Dia9{
    static  GrafoNoDirigo grafo = new GrafoNoDirigo();
    public static string Execute(){
        string[] input = Commons.ReadLines("Dia9/dia9.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part2(input);
    }

    private static string Part1(string[] input){

        foreach(string linea in input){
            string[] parse = linea.Split(" ");
            string o = parse[0];
            string d = parse[2];
            int c = int.Parse(linea.Split("= ")[1]);
            
            grafo.AddConnection(o,d,c);
        }
        return BruteForceShortestRouteCost(grafo).ToString();
    }

    private static string Part2(string[] input){
        return BruteForceLongestRouteCost(grafo).ToString();
    }

    public static int BruteForceShortestRouteCost(Grafo g){
        int min = int.MaxValue;
        foreach(string nodo in g.GetNodes()){
            List<string> nodes = new List<string>(g.GetNodes());
            nodes.Remove(nodo);
            List<List<string>> permutaciones = Permutar(nodes);

            foreach (var permutacion in permutaciones)
            {
                int distanciaActual = 0;
                string nodoActual = nodo;

                for (int i = 0; i < permutacion.Count; i++)
                {
                    if (g.Adjacent(nodoActual).ContainsKey(permutacion[i]))
                    {
                        distanciaActual += g.Adjacent(nodoActual)[permutacion[i]];
                        nodoActual = permutacion[i];
                    }
                    else
                    {
                        distanciaActual = int.MaxValue;
                        break;
                    }
                }
                min = Math.Min(distanciaActual, min);
            }
        }
        return min;
    }

    public static int BruteForceLongestRouteCost(Grafo g){
        int max = 0;
        foreach(string nodo in g.GetNodes()){
            List<string> nodes = new List<string>(g.GetNodes());
            nodes.Remove(nodo);
            List<List<string>> permutaciones = Permutar(nodes);

            foreach (var permutacion in permutaciones)
            {
                int distanciaActual = 0;
                string nodoActual = nodo;

                for (int i = 0; i < permutacion.Count; i++)
                {
                    if (g.Adjacent(nodoActual).ContainsKey(permutacion[i]))
                    {
                        distanciaActual += g.Adjacent(nodoActual)[permutacion[i]];
                        nodoActual = permutacion[i];
                    }
                    else
                    {
                        distanciaActual = int.MaxValue;
                        break;
                    }
                }
                max = Math.Max(distanciaActual, max);
            }
        }
        return max;
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

}