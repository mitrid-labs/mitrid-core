use base::Datable;

pub enum Request<StaP, StoP, RP, EP>
    where   StaP: Datable,
            StoP: Datable,
            RP: Datable,
            EP: Datable,
{
    Start { params: StaP },
    Stop { params: StoP },
    Restart { params: RP },
    Exec { params: EP },
}