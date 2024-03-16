
use async_trait::async_trait;
use pingora::prelude::*;
use std::sync::Arc;

pub struct LB(pub Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();

    fn new_ctx(&self) -> () {
        ()
    }

    async fn upstream_peer(&self, 
        _session: &mut Session, 
        _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self.0.select(b"", 256).unwrap();
        println!("upstream peer is: {:?}", upstream);
        // let addr = ("127.0.0.1", 3000);
        // let peer = Box::new(HttpPeer::new(addr, false, "127.0.0.1".to_string()));
// Set SNI to one.one.one.one
        let peer = Box::new(HttpPeer::new(upstream.addr, false, "1.1.1.1".to_string()));
      
        Ok(peer)
    }

    // async fn upstream_request_filter(
    //     &self,
    //     _session: &mut Session,
    //     upstream_request: &mut RequestHeader,
    //     _ctx: &mut Self::CTX,
    // ) -> Result<()> {
    //     println!("upstream request filter: {:?}", upstream_request);
    //     upstream_request.insert_header("Host", "one.one.one.one").unwrap();
    //     Ok(())
    // }
    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX
    ) -> Result<bool> {
        println!("upstream request filter:");
        session
            .req_header_mut()
            .insert_header("Host", "127.0.0.1")
            .unwrap();
        Ok(false)
    }
}
