using commons;
class Dia23{    
    public static string Execute(){
        string[] input = Commons.ReadLines("Dia23/dia23.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part2(input);
    }

    private static string Part1(string[] input){
        List<Instruccion> instrucciones = new List<Instruccion>();
        uint a = 0, b = 0;
        foreach(string line in input){
            string l = line.Replace(",", "");
            string[] particiones = l.Split(" ");
            if(particiones.Length == 3){
                instrucciones.Add(new Instruccion(particiones[0], particiones[1], int.Parse(particiones[2])));
            }
            else if(particiones[0].StartsWith("j")){
                instrucciones.Add(new Instruccion(particiones[0],"", int.Parse(particiones[1])));
            }
            else{
                instrucciones.Add(new Instruccion(particiones[0], particiones[1], 0));
            }
        }

        int i = 0;
        while(i < instrucciones.Count && i >= 0){
            Instruccion inst = instrucciones[i];
            switch(inst.nombre){
                case "hlf":
                    if(inst.registro == "a"){a = a/2;}
                    else{b = b/2;}
                    i++;
                    break;
                case "tpl":
                    if(inst.registro == "a"){a = a*3;}
                    else{b = b*3;}
                    i++;
                    break;
                case "inc":
                    if(inst.registro == "a"){a++;}
                    else{b++;}
                    i++;
                break;
                case "jmp":
                    i += inst.offset;
                break;
                case "jie":
                    if((inst.registro == "a" ? a : b) % 2 == 0){i += inst.offset;}
                    else{i++;}
                break;
                case "jio":
                    if((inst.registro == "a" ? a : b) == 1){i += inst.offset;}
                    else{i++;}
                break;
            }
        }

        return b.ToString();
    }
     private static string Part2(string[] input){
        List<Instruccion> instrucciones = new List<Instruccion>();
        uint a = 1, b = 0;
        foreach(string line in input){
            string l = line.Replace(",", "");
            string[] particiones = l.Split(" ");
            if(particiones.Length == 3){
                instrucciones.Add(new Instruccion(particiones[0], particiones[1], int.Parse(particiones[2])));
            }
            else if(particiones[0].StartsWith("j")){
                instrucciones.Add(new Instruccion(particiones[0],"", int.Parse(particiones[1])));
            }
            else{
                instrucciones.Add(new Instruccion(particiones[0], particiones[1], 0));
            }
        }

        int i = 0;
        while(i < instrucciones.Count && i >= 0){
            Instruccion inst = instrucciones[i];
            switch(inst.nombre){
                case "hlf":
                    if(inst.registro == "a"){a = a/2;}
                    else{b = b/2;}
                    i++;
                    break;
                case "tpl":
                    if(inst.registro == "a"){a = a*3;}
                    else{b = b*3;}
                    i++;
                    break;
                case "inc":
                    if(inst.registro == "a"){a++;}
                    else{b++;}
                    i++;
                break;
                case "jmp":
                    i += inst.offset;
                break;
                case "jie":
                    if((inst.registro == "a" ? a : b) % 2 == 0){i += inst.offset;}
                    else{i++;}
                break;
                case "jio":
                    if((inst.registro == "a" ? a : b) == 1){i += inst.offset;}
                    else{i++;}
                break;
            }
        }

        return b.ToString();
    }

private struct Instruccion{
    public readonly string nombre;
    public readonly string registro;
    public readonly int offset;

    public Instruccion(string n, string r, int of){
        nombre = n;
        registro = r;
        offset = of;
    }

}

}