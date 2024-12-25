using commons;
class Dia2{
    public static string Execute(){
        string[] input = Commons.ReadLines("Dia2/dia2.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part2(input);
    }

    private static string Part1(string[] input){
        int cant = 0;
        foreach(string present in input){
            string[] sizes = present.Split("x");
            int l = int.Parse(sizes[0]);
            int w = int.Parse(sizes[1]);
            int h = int.Parse(sizes[2]);

            int lw = l * w;
            int wh = w * h;
            int hl = h * l;

            cant += (2 * (lw + wh + hl)) + Math.Min(Math.Min(lw, wh),hl);
            
        }
        return cant.ToString();
    }

    private static string Part2(string[] input){int cant = 0;
        foreach(string present in input){
            string[] sizes = present.Split("x");
            int l = int.Parse(sizes[0]);
            int w = int.Parse(sizes[1]);
            int h = int.Parse(sizes[2]);

            cant += 2 * (l + w + h - Math.Max(Math.Max(l, w),h)) + l*w*h;
            
        }
        return cant.ToString();
    }

}