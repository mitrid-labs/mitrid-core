use base::Datable;

pub enum Response<StaR, StoR, RR, ER>
    where   StaR: Datable,
            StoR: Datable,
            RR: Datable,
            ER: Datable,
{
    Start { result: Option<StaR>, error: Option<String> },
    Stop { result: Option<StoR>, error: Option<String> },
    Restart { result: Option<RR>, error: Option<String> },
    Exec { result: Option<ER>, error: Option<String> },
}