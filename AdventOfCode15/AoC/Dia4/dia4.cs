using commons;
class Dia4{
    public static string Execute(){
        string input = Commons.ReadInput("Dia4/dia4.txt");
        return "Primera parte: " + Part1(input) + "\nSegunda parte: " + Part2(input);
    }

    private static string Part1(string input){
        int i = 0;

        string hash;
        do{
            i++;
            hash = CreateMD5(input + i);
        }
        while(!hash.StartsWith("00000"));

        return i.ToString();
    }

    private static string Part2(string input){
        int i = 0;
        
        string hash;
        do{
            i++;
            hash = CreateMD5(input + i);
        }
        while(!hash.StartsWith("000000"));

        return i.ToString();
    }

    public static string CreateMD5(string input)
    {
        using (System.Security.Cryptography.MD5 md5 = System.Security.Cryptography.MD5.Create())
        {
            byte[] inputBytes = System.Text.Encoding.ASCII.GetBytes(input);
            byte[] hashBytes = md5.ComputeHash(inputBytes);

            return Convert.ToHexString(hashBytes);
        }
    }

}