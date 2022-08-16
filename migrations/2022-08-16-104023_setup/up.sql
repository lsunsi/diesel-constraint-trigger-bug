CREATE TABLE tabela(id INT PRIMARY KEY);

CREATE FUNCTION before_trigger() RETURNS TRIGGER
AS $before_trigger$
BEGIN
	IF NEW.id = 2 THEN
		RAISE EXCEPTION 'Failed before trigger.';
	END IF;

	RETURN NEW;
END;
$before_trigger$ LANGUAGE PLPGSQL;

CREATE FUNCTION after_trigger() RETURNS TRIGGER
AS $after_trigger$
BEGIN
   IF NEW.id = 3 THEN
		RAISE EXCEPTION 'Failed after trigger.';
	END IF;

	RETURN NEW;
END;
$after_trigger$ LANGUAGE PLPGSQL;

CREATE FUNCTION after_trigger_deferred() RETURNS TRIGGER
AS $after_trigger$
BEGIN
   IF NEW.id = 4 THEN
		RAISE EXCEPTION 'Failed after trigger deferred.';
	END IF;

	RETURN NEW;
END;
$after_trigger$ LANGUAGE PLPGSQL;

CREATE TRIGGER before_trigger
BEFORE INSERT ON tabela
FOR EACH ROW
EXECUTE FUNCTION before_trigger();

CREATE CONSTRAINT TRIGGER after_trigger
AFTER INSERT ON tabela
FOR EACH ROW
EXECUTE FUNCTION after_trigger();

CREATE CONSTRAINT TRIGGER after_trigger_deferred
AFTER INSERT ON tabela
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE FUNCTION after_trigger_deferred();
