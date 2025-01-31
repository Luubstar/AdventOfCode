using System.Diagnostics.CodeAnalysis;
using commons;
class Dia17{
    public static string Execute(){
        string[] input = Commons.ReadLines("Dia17/dia17.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part2(input);
    }

    private static string Part1(string[] input){
        List<int> contenedor = new List<int>(input.Select(int.Parse).ToArray());
        int v = Contar(contenedor, 150).Count;
        return v.ToString();
    }

    private static string Part2(string[] input){
        List<int> contenedor = new List<int>(input.Select(int.Parse).ToArray());
        List<List<int>> v = Contar(contenedor, 150);
        int min = int.MaxValue;
        foreach(List<int> c in v){min = Math.Min(min, c.Count);}

        int i = 0;
        foreach(List<int> c in v){if(c.Count==min){i++;}}

        return i.ToString();
    }

    private static void Contar(List<int> valores, int cap, List<int> combinacion, List<List<int>> result){
        if(cap == 0){result.Add(new List<int>(combinacion)); return;}
        if(valores.Count == 0 || cap < 0){return;}
        
        for (int i = 0; i < valores.Count; i++)
        {
            int current = valores[i];
            combinacion.Add(current);

            List<int> remainingContainers = valores.GetRange(i + 1, valores.Count - (i + 1));
            Contar(remainingContainers, cap - current, combinacion, result);

            combinacion.RemoveAt(combinacion.Count - 1);
        }

    }
    public static List<List<int>> Contar(List<int> valores, int cap){
        List<List<int>> result = new List<List<int>>();
        Contar(valores, cap, new List<int>(), result);
        return result;
    }

}