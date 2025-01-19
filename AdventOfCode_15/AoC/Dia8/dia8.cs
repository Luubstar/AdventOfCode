using commons;
class Dia8{
    public static string Execute(){
        string[] input = Commons.ReadLines("Dia8/dia8.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part2(input);
    }

    private static string Part1(string[] input){
        int totalCar = 0;
        int totalSpe = 0;
        
        foreach(string line in input){
            int car = 0;
            int spe = 2;

            for(int i = 1; i < line.Length-1; i++){
                if(line[i] == '\\'){
                    if(line[i + 1] == '"'){
                        i += 1;
                        spe++;
                    }
                    else if(line[i + 1] == 'x'){
                        i += 3;
                        spe += 3;
                    }
                    else{
                        i += 1;
                        spe++;
                    }
                }
                car++;
                spe++;
            }
            totalCar += car;
            totalSpe += spe;
        }

        return (totalSpe - totalCar).ToString();
    }

    private static string Part2(string[] input){
        int totalCar = 0;
        int totalSpe = 0;
        
        foreach(string line in input){
            int car = 6;
            int spe = 2;

            for(int i = 1; i < line.Length-1; i++){
                if(line[i] == '\\'){
                    if(line[i + 1] == '"'){
                        i += 1;
                        spe++;
                        car+=3;
                    }
                    else if(line[i + 1] == 'x'){
                        i += 3;
                        spe += 3;
                        car +=4;
                    }
                    else{
                        i += 1;
                        spe++;
                        car += 3;
                    }
                }
                car++;
                spe++;
            }
            totalCar += car;
            totalSpe += spe;
        }
        return (totalCar - totalSpe).ToString();
    }

}