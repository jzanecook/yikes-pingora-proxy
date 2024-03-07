use log::info;
use async_trait::async_trait;
use pingora::prelude::*;
use std::sync::Arc;
use structopt::StructOpt;

pub struct LB(Arc<LoadBalancer<RoundRobin>>, Arc<String>);

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) -> () {
        ()
    }

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self.0
            .select(b"", 256) // hash doesn't matter for round robin
            .unwrap();

        let sni = self.1
            .as_str();

        info!("upstream peer is: {:?}", upstream);
        info!("sni is: {:?}", sni);

        // Set SNI to one.one.one.one
        let peer = Box::new(HttpPeer::new(upstream, true, sni.to_string()));
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        let sni = self.1
            .as_str();
        upstream_request.insert_header("Host", sni).unwrap();
        Ok(())
    }
}



#[derive(StructOpt)]
struct YikesOpt {
    #[structopt(short)]
    sources: Vec<String>,

    #[structopt(short = "S", default_value = "localhost")]
    sni: String,

    #[structopt(short = "h", default_value = "0.0.0.0")]
    host: String,

    #[structopt(short = "p", default_value = "6188")]
    port: String,

    #[structopt(flatten)]
    base_opts: Opt,
}

fn main() {
    env_logger::init();

    let yikes_opts = YikesOpt::from_args();

    let mut my_server = Server::new(Some(yikes_opts.base_opts)).unwrap();

    info!("Configuration: {:?}", my_server.configuration);
    info!("Sources: {:?}", yikes_opts.sources);
    info!("Upstream SNI: {:?}", yikes_opts.sni);
    info!("Hosting at: {}:{}", yikes_opts.host, yikes_opts.port);

    my_server.bootstrap();

    let upstreams =
    LoadBalancer::try_from_iter(yikes_opts.sources).unwrap();

    let mut lb = http_proxy_service(
    &my_server.configuration, 
    LB(Arc::new(upstreams), Arc::new(yikes_opts.sni))
    );
    lb.add_tcp(&format!("{}:{}", yikes_opts.host, yikes_opts.port));

    my_server.add_service(lb);

    my_server.run_forever();
}
