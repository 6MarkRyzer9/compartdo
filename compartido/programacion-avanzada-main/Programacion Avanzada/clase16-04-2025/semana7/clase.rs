fn main (){
    let mut s=[5,2,4,6,1,3];
    for i in 1..6 {
        let pivote=s[i];
        let j=i-1;
        while j>0 && s[j-1]>pivote{
            s[j]=s[j-1];
            let _j=j-1;
        }
        s[j]=pivote;
        }
    }

