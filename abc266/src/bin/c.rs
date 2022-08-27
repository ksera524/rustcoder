use proconio::{fastout, input};
use proconio::marker::*;
#[fastout]
fn main() {
    input! {
        ax:f32,
        ay:f32,
        bx:f32,
        by:f32,
        cx:f32,
        cy:f32,
        dx:f32,
        dy:f32,
    }
        let ansd = isInside(ax,ay,bx,by,cx,cy,dx,dy);
        let ansc = isInside(ax,ay,bx,by,dx,dy,cx,cy);
        let ansb = isInside(ax,ay,dx,dy,cx,cy,bx,by);
        let ansa = isInside(dx,dy,bx,by,cx,cy,ax,ay);
    
    if(ansa && ansb && ansc && ansd){
        println!("{}", "Yes" );
    }else{
        println!("{}", "No" );
    }
}

fn isInside(ax:f32,
    ay:f32,
    bx:f32,
    by:f32,
    cx:f32,
    cy:f32,
    tx:f32,
    ty:f32) -> bool{
        let abXat = (bx-ax)*(ty-ay) - (by-ay)*(tx-ax);
        let bcXbt = (cx-bx)*(ty-by) - (cy-by)*(tx-bx);
        let caXct = (ax-cx)*(ty-cy) - (ay-cy)*(tx-cx);
    if(( abXat > 0.0 && bcXbt > 0.0 && caXct > 0.0) || ( abXat < 0.0 && bcXbt < 0.0 && caXct < 0.0))
        {
            return false;
        }
        else if(abXat*bcXbt*caXct == 0.0)
        {
            return true;
        }
        return true;
    }