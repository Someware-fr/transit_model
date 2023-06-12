FROM 162230498103.dkr.ecr.eu-west-1.amazonaws.com/mutable-proj-ci:latest-proj8.1.0 as builder
WORKDIR /usr/src/app
COPY . ./
RUN git describe --tags --always && git status
RUN cargo build --workspace --release
RUN mkdir /usr/src/bin && for file in ls ${CARGO_TARGET_DIR:-./target}/release/*; do if test -f $file -a -x $file; then cp $file /usr/src/bin; fi; done

FROM 162230498103.dkr.ecr.eu-west-1.amazonaws.com/mutable-proj-ci:8.1.0
COPY --from=builder /usr/src/bin/* /usr/local/bin/
