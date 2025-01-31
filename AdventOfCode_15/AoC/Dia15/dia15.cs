using commons;
class Dia15{
    public static string Execute(){
        string[] input = Commons.ReadLines("Dia15/dia15.txt");
        return "Primera parte: " + Part1(input);
    }

    private static string Part1(string[] input){
        List<Ingrediente> ingredientes = new List<Ingrediente>();

        foreach(string s in input){
            string r = s.Replace(",", "");
            string[] separadores = r.Split(" ");

            Ingrediente i = new Ingrediente(
                int.Parse(separadores[2]), int.Parse(separadores[4]),
                int.Parse(separadores[6]), int.Parse(separadores[8]),
                int.Parse(separadores[10])
            );

            ingredientes.Add(i);
        }

        return Repartir(ingredientes, 100).ToString() + "|" + RepartirLimitado(ingredientes, 100, 500).ToString();
    }


    static List<int[]> GeneratePermutations(int length, int totalUnits)
    {
        List<int[]> permutations = new List<int[]>();
        int[] current = new int[length];

        void Recurse(int index, int remainingUnits)
        {
            if (index == length - 1)
            {
                current[index] = remainingUnits;
                permutations.Add((int[])current.Clone());
                return;
            }

            for (int i = 0; i <= remainingUnits; i++)
            {
                current[index] = i;
                Recurse(index + 1, remainingUnits - i);
            }
        }

        Recurse(0, totalUnits);
        return permutations;
    }

    private static int Repartir(List<Ingrediente> l, int cucharadas){
        return RepartirLimitado(l, cucharadas, -1);
    }

    private static int RepartirLimitado(List<Ingrediente> l, int cucharadas, int c){
        List<int[]> combinaciones = GeneratePermutations(l.Count, cucharadas);

        int max = 0;

        foreach(int[] combinacion in combinaciones){
            int capacity = 0;
            int durability = 0;
            int flavor = 0;
            int texture = 0;
            int calories = 0;
            for(int i = 0; i < combinacion.Length; i++){
                Ingrediente g = l[i];

                capacity += g.capacity * combinacion[i];               
                durability += g.durability * combinacion[i];
                texture += g.texture * combinacion[i];
                flavor += g.flavor * combinacion[i];
                calories += g.calories * combinacion[i];

                if(calories > c && c != -1){break;}
            }
            
            capacity = Math.Max(0, capacity);
            durability = Math.Max(0, durability);
            texture = Math.Max(0, texture);
            flavor = Math.Max(0, flavor);

            if(c == -1 | calories == c) {max = Math.Max(max, capacity * flavor * durability * texture);}
        }
        return max;
    }
private struct Ingrediente{
    public readonly int capacity;
    public readonly int durability;
    public readonly int flavor;
    public readonly int texture;
    public readonly int calories;

    public Ingrediente(int c, int d, int f, int t, int ca){
        capacity = c;
        durability = d;
        flavor = f;
        texture = t;
        calories = ca;
    }
}
}

