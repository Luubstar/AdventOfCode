using commons;
class Dia24{    
    public static string Execute(){
        string[] input = Commons.ReadLines("Dia24/dia24.txt");
        int[] intinput = new int[input.Length];
        for(int i = 0; i < intinput.Length;i++){intinput[i] = int.Parse(input[i]);}
        return "Primera parte: " + Part1(intinput) + "\nSegunda parte: " + Part2(intinput);
    }

    private static string Part1(int[] input){
        List<int> lista = new List<int>(input);

        double i = Dividir(lista,lista.Sum()/3, 3, 3);
        return i.ToString();
    }
    private static string Part2(int[] input){
        List<int> lista = new List<int>(input);

        double i = Dividir(lista,lista.Sum()/4, 4, 4);
        return i.ToString();
    }

    public static double Dividir(List<int> lista, int suma, int sub, int partes){
        for(int y = 1; y <= lista.Count; y++){
            foreach(var combination in Combinations(lista, y)){
                if(combination.Sum() == suma){
                    if(sub == 1){
                        return 1;
                    }
                    else if (sub < partes){
                        var restante = lista.Except(combination).ToList();
                        return Dividir(restante, suma, sub-1, partes);
                    }
                    else{
                        var restante = lista.Except(combination).ToList();
                        double resultado = Dividir(restante, suma, sub-1, partes);
                        if(resultado > 0){
                            return combination.Aggregate(1.0, (acc,x) => acc *x);
                        }
                    }
                }

            }
       }
       return -1;
    }

    static IEnumerable<List<int>> Combinations(List<int> lst, int length)
    {
        if (length == 1)
        {
            return lst.Select(t => new List<int> { t });
        }

        return lst.SelectMany((t, i) => 
            Combinations(lst.Skip(i + 1).ToList(), length - 1)
            .Select(tl => new List<int> { t }.Concat(tl).ToList()));
    }


}