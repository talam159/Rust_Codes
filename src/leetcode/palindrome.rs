impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if(x<0 || (x%10==0 && x!=0)){
            return false;
        }
            let mut temp = x;
            let mut reverse_number :i32 = 0;
            while temp!=0{
                reverse_number= (temp%10)+reverse_number*10;
                temp=temp/10;
            }
            //return if reversed number is the same as the original number
        x==reverse_number
        

    }
}