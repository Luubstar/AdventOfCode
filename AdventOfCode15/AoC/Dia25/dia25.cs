using commons;
class Dia25{    
    public static string Execute(){
        (int, int) coord = (3083, 2978);
        return "Primera parte: " + Part1(coord) + "\nSegunda parte: " + Part2(coord);
    }

    private static string Part1((int x,int y) input){
        long pos = 1;
        uint ox = 1;
        uint oy = 1;
        ulong v = 20151125;
        while(true){
            if (oy == 1){
                oy = ox + 1;
                ox = 1;
                pos++;
            }
            else{
                oy -= 1;
                ox += 1;
                pos++;
            }
            
            v = (v*252533) % 33554393;
            if(ox == input.x && oy == input.y){break;}

        }

        return v.ToString();
    }
    private static string Part2((int,int)  input){
        return "";
    }

}