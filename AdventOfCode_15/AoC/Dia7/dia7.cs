using System.Reflection;
using commons;
class Dia7{
    public static string Execute(){
        string[] input = Commons.ReadLines("Dia7/dia7.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part2(input);
    }

    private static string Part1(string[] input){
        Dictionary<string, ushort> valores = new Dictionary<string, ushort>();
        Dictionary<string, string> instrucciones = new Dictionary<string, string>();

        foreach (string order in input)
        {
            string[] parts = order.Split(" -> ");
            instrucciones[parts[1]] = parts[0];
        }

        ushort Evaluar(string cable){
            if(valores.ContainsKey(cable)){return valores[cable];}

            string instruccion = instrucciones[cable];
            string[] segmentos = instruccion.Split(" ");

            ushort value;
            if(ushort.TryParse(segmentos[0], out value) && segmentos.Length == 1){valores[cable] = value;}
            else if(segmentos.Length == 1){valores[cable] = Evaluar(segmentos[0]);}
            else if(segmentos.Length == 2){valores[cable] = (ushort)~Evaluar(segmentos[1]);}
            else if(segmentos.Length == 3){

                ushort left;
                ushort right;
                
                if(segmentos[0] != "1"){left= Evaluar(segmentos[0]);}
                else{left = 1;}
                
                switch(segmentos[1]){
                    case "AND":
                        right = Evaluar(segmentos[2]);
                        valores[cable] = (ushort)(left & right);
                        break;
                    case "OR":
                        right = Evaluar(segmentos[2]);
                        valores[cable] = (ushort)(left | right);
                        break;
                    case "LSHIFT":
                        valores[cable] = (ushort)(left << int.Parse(segmentos[2]));
                        break;
                    case "RSHIFT":
                        valores[cable] = (ushort)(left >> int.Parse(segmentos[2]));
                        break;
                    default:
                        Console.WriteLine("error");
                        break;
                }

            }

            return valores[cable];
        }   

        return Evaluar("a").ToString();
    }

    private static string Part2(string[] input){
        Dictionary<string, ushort> valores = new Dictionary<string, ushort>();
        Dictionary<string, string> instrucciones = new Dictionary<string, string>();

        foreach (string order in input)
        {
            string[] parts = order.Split(" -> ");
            instrucciones[parts[1]] = parts[0];
        }

        valores["b"] = 46065;

        ushort Evaluar(string cable){
            if(valores.ContainsKey(cable)){return valores[cable];}

            string instruccion = instrucciones[cable];
            string[] segmentos = instruccion.Split(" ");

            ushort value;
            if(ushort.TryParse(segmentos[0], out value) && segmentos.Length == 1){valores[cable] = value;}
            else if(segmentos.Length == 1){valores[cable] = Evaluar(segmentos[0]);}
            else if(segmentos.Length == 2){valores[cable] = (ushort)~Evaluar(segmentos[1]);}
            else if(segmentos.Length == 3){

                ushort left;
                ushort right;
                
                if(segmentos[0] != "1"){left= Evaluar(segmentos[0]);}
                else{left = 1;}
                
                switch(segmentos[1]){
                    case "AND":
                        right = Evaluar(segmentos[2]);
                        valores[cable] = (ushort)(left & right);
                        break;
                    case "OR":
                        right = Evaluar(segmentos[2]);
                        valores[cable] = (ushort)(left | right);
                        break;
                    case "LSHIFT":
                        valores[cable] = (ushort)(left << int.Parse(segmentos[2]));
                        break;
                    case "RSHIFT":
                        valores[cable] = (ushort)(left >> int.Parse(segmentos[2]));
                        break;
                    default:
                        Console.WriteLine("error");
                        break;
                }

            }

            return valores[cable];
        }   

        return Evaluar("a").ToString();
    }

}