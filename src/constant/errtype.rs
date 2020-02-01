macro_rules! define_errcode {
    ( $( $name:ident = $code:expr), * ) => {
            #[derive(Debug,Eq,PartialEq,Clone)]
            pub enum ErrCode{
                $(
                    $name = $code,
                )*
            }

            impl From<u32> for ErrCode{
                fn from(code: u32)->Self{
                    $(
                        if code == $code{
                            return Self::$name;
                        }
                    )*
                    return Self::MetaError;
                }
            }
    };
}

define_errcode![ OperationNotAllowed = 3 ,
    OperationNotSupport = 4,
    USIMNotInseted = 10,
    USIMFailure = 13,
    USIMBusy=14,
    USIMMemoryFull = 20,
    MemoryFailure = 23,
    TextStringTooLong = 24,
    InvalidCharactersInTextString=25,
    NoNetWorkService=30,
    NetworkTimeout =31,
    NetworkNotAllowed =32,
    IncooectParameters=50,
    Unknown=100,

    //Quectel BC26 specific
    BC26PSDServicesNotAllowed = 107,
    PLMNNotAllowed = 111,
    //Error
    MetaError= 0 
];
